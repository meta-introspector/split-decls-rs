// Generated macro for Token (struct)
macro_rules! Depcrate_selectToken {
() => {
// Module: crate::select
// Provides: {"Token"}
// Dependencies: {}
# [doc = " Temporary data that gets initialized during select or a blocking operation, and is consumed by"] # [doc = " `read` or `write`."] # [doc = ""] # [doc = " Each field contains data associated with a specific channel flavor."] # [derive (Debug , Default)] pub struct Token { pub (crate) at : flavors :: at :: AtToken , pub (crate) array : flavors :: array :: ArrayToken , pub (crate) list : flavors :: list :: ListToken , # [allow (dead_code)] pub (crate) never : flavors :: never :: NeverToken , pub (crate) tick : flavors :: tick :: TickToken , pub (crate) zero : flavors :: zero :: ZeroToken , }
};
}
