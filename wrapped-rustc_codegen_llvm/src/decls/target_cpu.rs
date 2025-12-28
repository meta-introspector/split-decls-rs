macro_rules! target_cpu {
    () => {
        pub (crate) fn target_cpu (sess : & Session) -> & str { let cpu_name = sess . opts . cg . target_cpu . as_deref () . unwrap_or_else (| | & sess . target . cpu) ; handle_native (cpu_name) }
    };
}

target_cpu!();