macro_rules! macro_42 {
    () => {
        # [cfg (all (target_has_atomic = "ptr" , not (target_has_atomic = "32")))] compile_error ! ("currently all targets that support `AtomicPtr` also support `AtomicU32`") ;
    };
}

macro_42!();