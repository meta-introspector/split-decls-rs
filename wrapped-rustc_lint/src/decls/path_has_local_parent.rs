macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! path_has_local_parent {
    () => {
        deps!();
        # [doc = " Given a path, this checks if the if the parent resolution def id corresponds to"] # [doc = " the def id of the parent impl definition (the direct one and the outermost one)."] # [doc = ""] # [doc = " Given this path, we will look at the path (and ignore any generic args):"] # [doc = ""] # [doc = " ```text"] # [doc = "    std::convert::PartialEq<Foo<Bar>>"] # [doc = "    ^^^^^^^^^^^^^^^^^^^^^^^"] # [doc = " ```"] # [inline] fn path_has_local_parent (path : & Path < '_ > , cx : & LateContext < '_ > , impl_parent : DefId , outermost_impl_parent : Option < DefId > ,) -> bool { path . res . opt_def_id () . is_some_and (| did | did_has_local_parent (did , cx . tcx , impl_parent , outermost_impl_parent)) }
    };
}

path_has_local_parent!();