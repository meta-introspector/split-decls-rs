
macro_rules! ceil_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ceil in module {}", module_path!());
    };
}

mkfn!{
    ceil_introspect!();
    pub fn ceil (mut x : f64) -> f64 { unsafe { core :: arch :: asm ! ("fld qword ptr [{x}]" , "fstcw [{x}]" , "mov word ptr [{x} + 2], 0x0b7f" , "fldcw [{x} + 2]" , "frndint" , "fldcw [{x}]" , "fstp qword ptr [{x}]" , x = in (reg) & mut x , out ("st(0)") _ , out ("st(1)") _ , out ("st(2)") _ , out ("st(3)") _ , out ("st(4)") _ , out ("st(5)") _ , out ("st(6)") _ , out ("st(7)") _ , options (nostack) ,) ; } x }
}

macro_rules! floor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function floor in module {}", module_path!());
    };
}

mkfn!{
    floor_introspect!();
    pub fn floor (mut x : f64) -> f64 { unsafe { core :: arch :: asm ! ("fld qword ptr [{x}]" , "fstcw [{x}]" , "mov word ptr [{x} + 2], 0x077f" , "fldcw [{x} + 2]" , "frndint" , "fldcw [{x}]" , "fstp qword ptr [{x}]" , x = in (reg) & mut x , out ("st(0)") _ , out ("st(1)") _ , out ("st(2)") _ , out ("st(3)") _ , out ("st(4)") _ , out ("st(5)") _ , out ("st(6)") _ , out ("st(7)") _ , options (nostack) ,) ; } x }
}