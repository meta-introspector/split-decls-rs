
macro_rules! add_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add in module {}", module_path!());
    };
}

mkfn!{
    add_introspect!();
    # [cold] # [track_caller] pub (super) const fn add () -> ! { panic ! ("attempt to add with overflow") }
}

macro_rules! sub_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sub in module {}", module_path!());
    };
}

mkfn!{
    sub_introspect!();
    # [cold] # [track_caller] pub (super) const fn sub () -> ! { panic ! ("attempt to subtract with overflow") }
}

macro_rules! mul_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mul in module {}", module_path!());
    };
}

mkfn!{
    mul_introspect!();
    # [cold] # [track_caller] pub (super) const fn mul () -> ! { panic ! ("attempt to multiply with overflow") }
}

macro_rules! div_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function div in module {}", module_path!());
    };
}

mkfn!{
    div_introspect!();
    # [cold] # [track_caller] pub (super) const fn div () -> ! { panic ! ("attempt to divide with overflow") }
}

macro_rules! rem_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rem in module {}", module_path!());
    };
}

mkfn!{
    rem_introspect!();
    # [cold] # [track_caller] pub (super) const fn rem () -> ! { panic ! ("attempt to calculate the remainder with overflow") }
}

macro_rules! neg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function neg in module {}", module_path!());
    };
}

mkfn!{
    neg_introspect!();
    # [cold] # [track_caller] pub (super) const fn neg () -> ! { panic ! ("attempt to negate with overflow") }
}

macro_rules! shr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function shr in module {}", module_path!());
    };
}

mkfn!{
    shr_introspect!();
    # [cold] # [track_caller] pub (super) const fn shr () -> ! { panic ! ("attempt to shift right with overflow") }
}

macro_rules! shl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function shl in module {}", module_path!());
    };
}

mkfn!{
    shl_introspect!();
    # [cold] # [track_caller] pub (super) const fn shl () -> ! { panic ! ("attempt to shift left with overflow") }
}