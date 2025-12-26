use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Marks async function to be executed by runtime, suitable to test environment
///
/// ## Usage
///
/// ```no_run
/// #[tokio::test]
/// async fn my_test() {
///     assert!(true);
/// }
/// ```
#[proc_macro_attribute]
pub fn test_rt(args: TokenStream, item: TokenStream) -> TokenStream {
    entry::test(args.into(), item.into(), false).into()
}
