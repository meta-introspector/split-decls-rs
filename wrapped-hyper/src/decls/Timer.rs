macro_rules! deps {
    () => {
        Sleep!();
    };
}

macro_rules! Timer {
    () => {
        deps!();
        # [doc = " A timer which provides timer-like functions."] pub trait Timer { # [doc = " Return a future that resolves in `duration` time."] fn sleep (& self , duration : Duration) -> Pin < Box < dyn Sleep > > ; # [doc = " Return a future that resolves at `deadline`."] fn sleep_until (& self , deadline : Instant) -> Pin < Box < dyn Sleep > > ; # [doc = " Return an `Instant` representing the current time."] # [doc = ""] # [doc = " The default implementation returns [`Instant::now()`]."] fn now (& self) -> Instant { Instant :: now () } # [doc = " Reset a future to resolve at `new_deadline` instead."] fn reset (& self , sleep : & mut Pin < Box < dyn Sleep > > , new_deadline : Instant) { * sleep = self . sleep_until (new_deadline) ; } }
    };
}

Timer!()