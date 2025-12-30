// Generated macro for InputStringKind (enum)
macro_rules! DepcrateInputStringKind {
() => {
// Module: crate
// Provides: {"InputStringKind"}
// Dependencies: {}
# [doc = " Whether the input string is a literal. If yes, it contains the inner width mappings."] # [derive (Clone , PartialEq , Eq)] enum InputStringKind { NotALiteral , Literal { width_mappings : Vec < InnerWidthMapping > , } , }
};
}
