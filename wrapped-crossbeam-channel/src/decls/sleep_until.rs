macro_rules! sleep_until {
    () => {
        # [doc = " Sleeps until the deadline, or forever if the deadline isn't specified."] pub (crate) fn sleep_until (deadline : Option < Instant >) { loop { match deadline { None => thread :: sleep (Duration :: from_secs (1000)) , Some (d) => { let now = Instant :: now () ; if now >= d { break ; } thread :: sleep (d - now) ; } } } }
    };
}

sleep_until!()