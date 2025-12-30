// Generated macro for len (function)
macro_rules! Depcrate_matchers_len_matcherlen {
() => {
// Module: crate::matchers::len_matcher
// Provides: {"len"}
// Dependencies: {}
# [doc = " Matches a container whose number of elements matches `expected`."] # [doc = ""] # [doc = " This matches against a container over which one can iterate. This includes"] # [doc = " the standard Rust containers, arrays, and slices. More"] # [doc = " precisely, the actual type must implement [`IntoIterator`]."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # fn should_pass() -> Result<()> {"] # [doc = " let array = [1,2,3];"] # [doc = " verify_that!(array, len(eq(3)))?;"] # [doc = " let vec = vec![1,2,3];"] # [doc = " verify_that!(vec, len(eq(3)))?;"] # [doc = " let slice = vec.as_slice();"] # [doc = " verify_that!(*slice, len(eq(3)))?;"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass().unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " The parameter `expected` can be any integer numeric matcher."] # [doc = ""] # [doc = " ```"] # [doc = " # use googletest::prelude::*;"] # [doc = " # fn should_pass() -> Result<()> {"] # [doc = " let vec = vec![1,2,3];"] # [doc = " verify_that!(vec, len(gt(1)))?;"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " # should_pass().unwrap();"] # [doc = " ```"] pub fn len < E > (expected : E) -> LenMatcher < E > { LenMatcher { expected } }
};
}
