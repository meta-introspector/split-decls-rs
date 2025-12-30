// Generated macro for prompt (function)
macro_rules! Depcrateprompt {
() => {
// Module: crate
// Provides: {"prompt"}
// Dependencies: {}
# [doc = " Calls the `prompt` function."] # [doc = ""] # [doc = " A default value can be supplied which will be returned if the user doesn't input anything."] # [doc = " This function will return `None` if the value of `default` is `None` and the user cancels"] # [doc = " the operation."] # [doc = ""] # [doc = " [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)"] pub fn prompt (message : & str , default : Option < & str >) -> Option < String > { match default { Some (default) => window () . prompt_with_message_and_default (message , default) . expect_throw ("can't read input") , None => window () . prompt_with_message (message) . expect_throw ("can't read input") , } }
};
}
