mkuse!{use std :: collections :: HashMap ;}
mkuse!{use test :: Bencher ;}

macro_rules! new_drop_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_drop in module {}", module_path!());
    };
}

mkfn!{
    new_drop_introspect!();
    # [bench] fn new_drop (b : & mut Bencher) { b . iter (| | { let m : HashMap < i32 , i32 > = HashMap :: new () ; assert_eq ! (m . len () , 0) ; }) }
}

macro_rules! new_insert_drop_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_insert_drop in module {}", module_path!());
    };
}

mkfn!{
    new_insert_drop_introspect!();
    # [bench] fn new_insert_drop (b : & mut Bencher) { b . iter (| | { let mut m = HashMap :: new () ; m . insert (0 , 0) ; assert_eq ! (m . len () , 1) ; }) }
}

macro_rules! grow_by_insertion_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function grow_by_insertion in module {}", module_path!());
    };
}

mkfn!{
    grow_by_insertion_introspect!();
    # [bench] fn grow_by_insertion (b : & mut Bencher) { let mut m = HashMap :: new () ; for i in 1 .. 1001 { m . insert (i , i) ; } let mut k = 1001 ; b . iter (| | { m . insert (k , k) ; k += 1 ; }) ; }
}

macro_rules! find_existing_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_existing in module {}", module_path!());
    };
}

mkfn!{
    find_existing_introspect!();
    # [bench] fn find_existing (b : & mut Bencher) { let mut m = HashMap :: new () ; for i in 1 .. 1001 { m . insert (i , i) ; } b . iter (| | { for i in 1 .. 1001 { m . contains_key (& i) ; } }) ; }
}

macro_rules! find_nonexisting_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_nonexisting in module {}", module_path!());
    };
}

mkfn!{
    find_nonexisting_introspect!();
    # [bench] fn find_nonexisting (b : & mut Bencher) { let mut m = HashMap :: new () ; for i in 1 .. 1001 { m . insert (i , i) ; } b . iter (| | { for i in 1001 .. 2001 { m . contains_key (& i) ; } }) ; }
}

macro_rules! hashmap_as_queue_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hashmap_as_queue in module {}", module_path!());
    };
}

mkfn!{
    hashmap_as_queue_introspect!();
    # [bench] fn hashmap_as_queue (b : & mut Bencher) { let mut m = HashMap :: new () ; for i in 1 .. 1001 { m . insert (i , i) ; } let mut k = 1 ; b . iter (| | { m . remove (& k) ; m . insert (k + 1000 , k + 1000) ; k += 1 ; }) ; }
}

macro_rules! get_remove_insert_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_remove_insert in module {}", module_path!());
    };
}

mkfn!{
    get_remove_insert_introspect!();
    # [bench] fn get_remove_insert (b : & mut Bencher) { let mut m = HashMap :: new () ; for i in 1 .. 1001 { m . insert (i , i) ; } let mut k = 1 ; b . iter (| | { m . get (& (k + 400)) ; m . get (& (k + 2000)) ; m . remove (& k) ; m . insert (k + 1000 , k + 1000) ; k += 1 ; }) }
}