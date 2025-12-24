use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A tuple returned by the validation and processing of the parsed grammar.
/// The first element is the vector of used builtin rule names,
/// the second element is the vector of optimized rules.
type UsedBuiltinAndOptimized<'i> = (Vec<&'i str>, Vec<optimizer::OptimizedRule>);
