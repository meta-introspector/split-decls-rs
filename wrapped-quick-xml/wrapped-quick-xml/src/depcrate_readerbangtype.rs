// Generated macro for BangType (enum)
macro_rules! Depcrate_readerBangType {
() => {
// Module: crate::reader
// Provides: {"BangType"}
// Dependencies: {}
# [doc = " Possible elements started with `<!`"] # [derive (Debug , PartialEq)] enum BangType { # [doc = " <![CDATA[...]]>"] CData , # [doc = " <!--...-->"] Comment , # [doc = " <!DOCTYPE...>. Contains balance of '<' (+1) and '>' (-1)"] DocType (i32) , }
};
}
