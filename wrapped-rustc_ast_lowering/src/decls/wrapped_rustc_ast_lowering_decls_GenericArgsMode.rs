use serde::{Deserialize, Serialize};
use std::collections::HashMap;
enum GenericArgsMode {
    /// Allow paren sugar, don't allow RTN.
    ParenSugar,
    /// Allow RTN, don't allow paren sugar.
    ReturnTypeNotation,
    Err,
    /// Silence errors when lowering generics. Only used with `Res::Err`.
    Silence,
}
