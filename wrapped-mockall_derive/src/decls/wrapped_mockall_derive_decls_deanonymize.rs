use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Replace any references to the anonymous lifetime `'_` with `'static`.
fn deanonymize(literal_type: &mut Type) {
    match literal_type {
        Type::Array(ta) => deanonymize(ta.elem.as_mut()),
        Type::BareFn(tbf) => {
            if let ReturnType::Type(_, ref mut bt) = tbf.output {
                deanonymize(bt.as_mut());
            }
            for input in tbf.inputs.iter_mut() {
                deanonymize(&mut input.ty);
            }
        }
        Type::Group(tg) => deanonymize(tg.elem.as_mut()),
        Type::Infer(_) => {}
        Type::Never(_) => {}
        Type::Paren(tp) => deanonymize(tp.elem.as_mut()),
        Type::Path(tp) => {
            if let Some(ref mut qself) = tp.qself {
                deanonymize(qself.ty.as_mut());
            }
            deanonymize_path(&mut tp.path);
        }
        Type::Ptr(tptr) => deanonymize(tptr.elem.as_mut()),
        Type::Reference(tr) => {
            if let Some(lt) = tr.lifetime.as_mut() {
                deanonymize_lifetime(lt)
            }
            deanonymize(tr.elem.as_mut());
        }
        Type::Slice(s) => deanonymize(s.elem.as_mut()),
        Type::TraitObject(tto) => {
            for tpb in tto.bounds.iter_mut() {
                match tpb {
                    TypeParamBound::Trait(tb) => deanonymize_path(&mut tb.path),
                    TypeParamBound::Lifetime(lt) => deanonymize_lifetime(lt),
                    _ => {}
                }
            }
        }
        Type::Tuple(tt) => {
            for ty in tt.elems.iter_mut() {
                deanonymize(ty)
            }
        }
        x => compile_error(x.span(), "Unimplemented type for deanonymize"),
    }
}
