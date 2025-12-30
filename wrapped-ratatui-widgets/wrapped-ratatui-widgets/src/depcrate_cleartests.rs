// Generated macro for tests (module)
macro_rules! Depcrate_cleartests {
() => {
// Module: crate::clear
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use ratatui_core :: buffer :: Buffer ; use ratatui_core :: layout :: Rect ; use ratatui_core :: widgets :: Widget ; use super :: * ; # [test] fn render () { let mut buffer = Buffer :: with_lines (["xxxxxxxxxxxxxxx" ; 7]) ; let clear = Clear ; clear . render (Rect :: new (1 , 2 , 3 , 4) , & mut buffer) ; let expected = Buffer :: with_lines (["xxxxxxxxxxxxxxx" , "xxxxxxxxxxxxxxx" , "x   xxxxxxxxxxx" , "x   xxxxxxxxxxx" , "x   xxxxxxxxxxx" , "x   xxxxxxxxxxx" , "xxxxxxxxxxxxxxx" ,]) ; assert_eq ! (buffer , expected) ; } }
};
}
