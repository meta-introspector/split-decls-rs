use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// How relaxed bounds `?Trait` should be treated.
///
/// Relaxed bounds should only be allowed in places where we later
/// (namely during HIR ty lowering) perform *sized elaboration*.
#[derive(Clone, Copy, Debug)]
enum RelaxedBoundPolicy<'a> {
    Allowed,
    AllowedIfOnTyParam(NodeId, &'a [ast::GenericParam]),
    Forbidden(RelaxedBoundForbiddenReason),
}
