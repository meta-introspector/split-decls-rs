use serde::{Deserialize, Serialize};
use std::collections::HashMap;
struct DynamicConfig<
    'tcx,
    C: QueryCache,
    const ANON: bool,
    const DEPTH_LIMIT: bool,
    const FEEDABLE: bool,
> {
    dynamic: &'tcx DynamicQuery<'tcx, C>,
}
