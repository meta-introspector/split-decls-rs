mkuse!{use std :: panic :: Location ;}

macro_rules! tracked_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tracked in module {}", module_path!());
    };
}

mkfn!{
    tracked_introspect!();
    # [track_caller] fn tracked () -> & 'static Location < 'static > { Location :: caller () }
}

macro_rules! nested_intrinsic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nested_intrinsic in module {}", module_path!());
    };
}

mkfn!{
    nested_intrinsic_introspect!();
    fn nested_intrinsic () -> & 'static Location < 'static > { Location :: caller () }
}

macro_rules! nested_tracked_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nested_tracked in module {}", module_path!());
    };
}

mkfn!{
    nested_tracked_introspect!();
    fn nested_tracked () -> & 'static Location < 'static > { tracked () }
}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { let location = Location :: caller () ; assert_eq ! (location . file () , file ! ()) ; assert_eq ! (location . line () , 21) ; assert_eq ! (location . column () , 20) ; let tracked = tracked () ; assert_eq ! (tracked . file () , file ! ()) ; assert_eq ! (tracked . line () , 26) ; assert_eq ! (tracked . column () , 19) ; let nested = nested_intrinsic () ; assert_eq ! (nested . file () , file ! ()) ; assert_eq ! (nested . line () , 13) ; assert_eq ! (nested . column () , 5) ; let contained = nested_tracked () ; assert_eq ! (contained . file () , file ! ()) ; assert_eq ! (contained . line () , 17) ; assert_eq ! (contained . column () , 5) ; }
}