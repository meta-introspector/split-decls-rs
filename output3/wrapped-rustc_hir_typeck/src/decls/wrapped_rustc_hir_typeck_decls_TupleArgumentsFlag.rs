use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Controls whether the arguments are tupled. This is used for the call
/// operator.
///
/// Tupling means that all call-side arguments are packed into a tuple and
/// passed as a single parameter. For example, if tupling is enabled, this
/// function:
/// ```
/// fn f(x: (isize, isize)) {}
/// ```
/// Can be called as:
/// ```ignore UNSOLVED (can this be done in user code?)
/// # fn f(x: (isize, isize)) {}
/// f(1, 2);
/// ```
/// Instead of:
/// ```
/// # fn f(x: (isize, isize)) {}
/// f((1, 2));
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
enum TupleArgumentsFlag {
    DontTupleArguments,
    TupleArguments,
}
