// Generated macro for Match (enum)
macro_rules! DepcrateMatch {
() => {
// Module: crate
// Provides: {"Match"}
// Dependencies: {}
# [doc = " The result of a glob match."] # [doc = ""] # [doc = " The type parameter `T` typically refers to a type that provides more"] # [doc = " information about a particular match. For example, it might identify"] # [doc = " the specific gitignore file and the specific glob pattern that caused"] # [doc = " the match."] # [derive (Clone , Debug)] pub enum Match < T > { # [doc = " The path didn't match any glob."] None , # [doc = " The highest precedent glob matched indicates the path should be"] # [doc = " ignored."] Ignore (T) , # [doc = " The highest precedent glob matched indicates the path should be"] # [doc = " whitelisted."] Whitelist (T) , }
};
}
