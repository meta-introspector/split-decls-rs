macro_rules! RaceOk {
    () => {
        # [doc = " Wait for the first successful future to complete."] # [doc = ""] # [doc = " Awaits multiple futures simultaneously, returning the output of the first"] # [doc = " future which completes successfully. If no future completes successfully,"] # [doc = " returns an aggregate error of all failed futures."] pub trait RaceOk { # [doc = " The resulting output type."] type Output ; # [doc = " The resulting error type."] type Error ; # [doc = " Which kind of future are we turning this into?"] type Future : Future < Output = Result < Self :: Output , Self :: Error > > ; # [doc = " Waits for the first successful future to complete."] fn race_ok (self) -> Self :: Future ; }
    };
}

RaceOk!()