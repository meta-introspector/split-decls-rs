use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Add `levels` `super::` to the path.  Return the number of levels added.
fn supersuperfy_path(path: &mut Path, levels: usize) -> usize {
    if let Some(t) = path.segments.last_mut() {
        match &mut t.arguments {
            PathArguments::None => {}
            PathArguments::AngleBracketed(ref mut abga) => {
                for arg in abga.args.iter_mut() {
                    match arg {
                        GenericArgument::Type(ref mut ty) => {
                            *ty = supersuperfy(ty, levels);
                        }
                        GenericArgument::AssocType(ref mut at) => {
                            at.ty = supersuperfy(&at.ty, levels);
                        }
                        GenericArgument::Constraint(ref mut constraint) => {
                            supersuperfy_bounds(&mut constraint.bounds, levels);
                        }
                        _ => {}
                    }
                }
            }
            PathArguments::Parenthesized(ref mut pga) => {
                for input in pga.inputs.iter_mut() {
                    *input = supersuperfy(input, levels);
                }
                if let ReturnType::Type(_, ref mut ty) = pga.output {
                    **ty = supersuperfy(ty, levels);
                }
            }
        }
    }
    if let Some(t) = path.segments.first() {
        if t.ident == "super" {
            let mut ident = format_ident!("super");
            ident.set_span(path.segments.span());
            let ps = PathSegment {
                ident,
                arguments: PathArguments::None,
            };
            for _ in 0..levels {
                path.segments.insert(0, ps.clone());
            }
            levels
        } else {
            0
        }
    } else {
        0
    }
}
