// Generated macro for render (function)
macro_rules! Depcraterender {
() => {
// Module: crate
// Provides: {"render"}
// Dependencies: {}
# [doc = " Renders graph `g` into the writer `w` in DOT syntax."] # [doc = " (Simple wrapper around `render_opts` that passes a default set of options.)"] pub fn render < 'a , N : Clone + 'a , E : Clone + 'a , G : Labeller < 'a , N , E > + GraphWalk < 'a , N , E > , W : Write , > (g : & 'a G , w : & mut W ,) -> io :: Result < () > { render_opts (g , w , & []) }
};
}
