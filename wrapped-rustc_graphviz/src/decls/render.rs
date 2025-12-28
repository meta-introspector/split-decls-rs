macro_rules! deps {
    () => {
        Node!();
        Labeller!();
        Edge!();
        GraphWalk!();
    };
}

macro_rules! render {
    () => {
        deps!();
        # [doc = " Renders directed graph `g` into the writer `w` in DOT syntax."] # [doc = " (Simple wrapper around `render_opts` that passes a default set of options.)"] pub fn render < 'a , N , E , G , W > (g : & 'a G , w : & mut W) -> io :: Result < () > where N : Clone + 'a , E : Clone + 'a , G : Labeller < 'a , Node = N , Edge = E > + GraphWalk < 'a , Node = N , Edge = E > , W : Write , { render_opts (g , w , & []) }
    };
}

render!()