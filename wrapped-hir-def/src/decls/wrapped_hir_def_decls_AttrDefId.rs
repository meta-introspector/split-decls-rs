use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AttrDefId {
    ModuleId(ModuleId),
    FieldId(FieldId),
    AdtId(AdtId),
    FunctionId(FunctionId),
    EnumVariantId(EnumVariantId),
    StaticId(StaticId),
    ConstId(ConstId),
    TraitId(TraitId),
    TypeAliasId(TypeAliasId),
    MacroId(MacroId),
    ImplId(ImplId),
    GenericParamId(GenericParamId),
    ExternBlockId(ExternBlockId),
    ExternCrateId(ExternCrateId),
    UseId(UseId),
}
