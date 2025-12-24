use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Enum {
    pub fn module(self, db: &dyn HirDatabase) -> Module {
        Module {
            id: self.id.lookup(db).container,
        }
    }
    pub fn name(self, db: &dyn HirDatabase) -> Name {
        db.enum_signature(self.id).name.clone()
    }
    pub fn variants(self, db: &dyn HirDatabase) -> Vec<Variant> {
        self.id
            .enum_variants(db)
            .variants
            .iter()
            .map(|&(id, _, _)| Variant { id })
            .collect()
    }
    pub fn num_variants(self, db: &dyn HirDatabase) -> usize {
        self.id.enum_variants(db).variants.len()
    }
    pub fn repr(self, db: &dyn HirDatabase) -> Option<ReprOptions> {
        db.enum_signature(self.id).repr
    }
    pub fn ty<'db>(self, db: &'db dyn HirDatabase) -> Type<'db> {
        Type::from_def(db, self.id)
    }
    pub fn ty_params<'db>(self, db: &'db dyn HirDatabase) -> Type<'db> {
        Type::from_def_params(db, self.id)
    }
    /// The type of the enum variant bodies.
    pub fn variant_body_ty<'db>(self, db: &'db dyn HirDatabase) -> Type<'db> {
        let interner = DbInterner::new_with(db, None, None);
        Type::new_for_crate(
            self.id.lookup(db).container.krate(),
            match db.enum_signature(self.id).variant_body_type() {
                layout::IntegerType::Pointer(sign) => {
                    match sign {
                        true => Ty::new_int(interner, rustc_type_ir::IntTy::Isize),
                        false => Ty::new_uint(interner, rustc_type_ir::UintTy::Usize),
                    }
                }
                layout::IntegerType::Fixed(i, sign) => {
                    match sign {
                        true => {
                            Ty::new_int(
                                interner,
                                match i {
                                    layout::Integer::I8 => rustc_type_ir::IntTy::I8,
                                    layout::Integer::I16 => rustc_type_ir::IntTy::I16,
                                    layout::Integer::I32 => rustc_type_ir::IntTy::I32,
                                    layout::Integer::I64 => rustc_type_ir::IntTy::I64,
                                    layout::Integer::I128 => rustc_type_ir::IntTy::I128,
                                },
                            )
                        }
                        false => {
                            Ty::new_uint(
                                interner,
                                match i {
                                    layout::Integer::I8 => rustc_type_ir::UintTy::U8,
                                    layout::Integer::I16 => rustc_type_ir::UintTy::U16,
                                    layout::Integer::I32 => rustc_type_ir::UintTy::U32,
                                    layout::Integer::I64 => rustc_type_ir::UintTy::U64,
                                    layout::Integer::I128 => rustc_type_ir::UintTy::U128,
                                },
                            )
                        }
                    }
                }
            },
        )
    }
    /// Returns true if at least one variant of this enum is a non-unit variant.
    pub fn is_data_carrying(self, db: &dyn HirDatabase) -> bool {
        self.variants(db).iter().any(|v| !matches!(v.kind(db), StructKind::Unit))
    }
    pub fn layout(self, db: &dyn HirDatabase) -> Result<Layout, LayoutError> {
        Adt::from(self).layout(db)
    }
    pub fn is_unstable(self, db: &dyn HirDatabase) -> bool {
        db.attrs(self.id.into()).is_unstable()
    }
}
