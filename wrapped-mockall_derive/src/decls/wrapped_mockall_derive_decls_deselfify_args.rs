use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Change any `Self` in a method's arguments' types with `actual`.
/// `generics` is the Generics field of the parent struct.
fn deselfify_args(
    args: &mut Punctuated<FnArg, Token![,]>,
    actual: &Ident,
    generics: &Generics,
) {
    for arg in args.iter_mut() {
        match arg {
            FnArg::Receiver(r) => {
                if r.colon_token.is_some() {
                    deselfify(r.ty.as_mut(), actual, generics)
                }
            }
            FnArg::Typed(pt) => deselfify(pt.ty.as_mut(), actual, generics),
        }
    }
}
