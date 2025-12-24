use serde::{Deserialize, Serialize};
use std::collections::HashMap;
type ChallengeParamRef<'i> = (&'i str, ParamValue<'i>);
