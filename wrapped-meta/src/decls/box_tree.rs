macro_rules! box_tree {
    () => {
        # [cfg (test)] macro_rules ! box_tree { ($ node : ident ($ ($ child : ident ($ ($ args : tt) *)) ,+)) => ($ node ($ (Box :: new (box_tree ! ($ child ($ ($ args) *)))) ,+)) ; ($ expr : expr) => ($ expr) ; }
    };
}

box_tree!()