mkuse!{use std :: collections :: HashSet ;}
mkuse!{use test :: Bencher ;}

macro_rules! set_difference_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_difference in module {}", module_path!());
    };
}

mkfn!{
    set_difference_introspect!();
    # [bench] fn set_difference (b : & mut Bencher) { let small : HashSet < _ > = (0 .. 10) . collect () ; let large : HashSet < _ > = (0 .. 100) . collect () ; b . iter (| | small . difference (& large) . count ()) ; }
}

macro_rules! set_is_subset_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_is_subset in module {}", module_path!());
    };
}

mkfn!{
    set_is_subset_introspect!();
    # [bench] fn set_is_subset (b : & mut Bencher) { let small : HashSet < _ > = (0 .. 10) . collect () ; let large : HashSet < _ > = (0 .. 100) . collect () ; b . iter (| | small . is_subset (& large)) ; }
}

macro_rules! set_intersection_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_intersection in module {}", module_path!());
    };
}

mkfn!{
    set_intersection_introspect!();
    # [bench] fn set_intersection (b : & mut Bencher) { let small : HashSet < _ > = (0 .. 10) . collect () ; let large : HashSet < _ > = (0 .. 100) . collect () ; b . iter (| | small . intersection (& large) . count ()) ; }
}

macro_rules! set_symmetric_difference_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_symmetric_difference in module {}", module_path!());
    };
}

mkfn!{
    set_symmetric_difference_introspect!();
    # [bench] fn set_symmetric_difference (b : & mut Bencher) { let small : HashSet < _ > = (0 .. 10) . collect () ; let large : HashSet < _ > = (0 .. 100) . collect () ; b . iter (| | small . symmetric_difference (& large) . count ()) ; }
}

macro_rules! set_union_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_union in module {}", module_path!());
    };
}

mkfn!{
    set_union_introspect!();
    # [bench] fn set_union (b : & mut Bencher) { let small : HashSet < _ > = (0 .. 10) . collect () ; let large : HashSet < _ > = (0 .. 100) . collect () ; b . iter (| | small . union (& large) . count ()) ; }
}