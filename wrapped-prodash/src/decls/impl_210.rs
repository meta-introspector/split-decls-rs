macro_rules! deps {
    () => {
        State!();
        Throughput!();
        Duration!();
        Step!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl State { fn new (value : progress :: Step , elapsed : Duration) -> Self { State { observed : elapsed , last_value : value , elapsed_values : { let mut v = VecDeque :: with_capacity (6) ; v . push_back ((elapsed , value)) ; v } , last_update_duration : elapsed , precomputed_throughput : None , } } fn compute_throughput (& mut self) -> progress :: Step { let mut observed : Duration = self . elapsed_values . iter () . map (| e | e . 0) . sum () ; while ! self . elapsed_values . is_empty () && observed > ONCE_A_SECOND { let candidate = self . elapsed_values . front () . map (| e | e . 0) . expect ("at least one item as we are in the checked loop") ; if observed . checked_sub (candidate) . unwrap_or_default () <= ONCE_A_SECOND { break ; } observed -= candidate ; self . elapsed_values . pop_front () ; } let observed_value : progress :: Step = self . elapsed_values . iter () . map (| e | e . 1) . sum () ; ((observed_value as f64 / observed . as_secs_f64 ()) * ONCE_A_SECOND . as_secs_f64 ()) as progress :: Step } fn update (& mut self , value : progress :: Step , elapsed : Duration) -> Option < unit :: display :: Throughput > { self . observed += elapsed ; self . elapsed_values . push_back ((elapsed , value . saturating_sub (self . last_value))) ; self . last_value = value ; if self . observed - self . last_update_duration > THROTTLE_INTERVAL { self . precomputed_throughput = Some (self . compute_throughput ()) ; self . last_update_duration = self . observed ; } self . throughput () } fn throughput (& self) -> Option < unit :: display :: Throughput > { self . precomputed_throughput . map (| tp | unit :: display :: Throughput { value_change_in_timespan : tp , timespan : ONCE_A_SECOND , }) } }
    };
}

impl_210!()