macro_rules! deps {
    () => {
        RawStderr!();
    };
}

macro_rules! other_55 {
    () => {
        deps!();
        # [doc = " We don't really care how many bytes we actually get out. SIGSEGV comes for our head."] # [doc = " Splash stderr with letters of our own blood to warn our friends about the monster."] macro raw_errln ($ tokens : tt) { let _ = :: core :: fmt :: Write :: write_fmt (& mut RawStderr (()) , format_args ! ($ tokens)) ; let _ = :: core :: fmt :: Write :: write_char (& mut RawStderr (()) , '\n') ; }
    };
}

other_55!()