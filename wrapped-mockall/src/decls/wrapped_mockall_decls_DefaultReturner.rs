use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Default)]
#[doc(hidden)]
pub struct DefaultReturner<O>(PhantomData<O>);
