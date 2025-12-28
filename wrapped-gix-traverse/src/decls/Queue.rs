macro_rules! deps {
    () => {
        Info!();
        Topo!();
    };
}

macro_rules! Queue {
    () => {
        deps!();
        # [derive (Debug)] pub (in crate :: commit) enum Queue { Date (PriorityQueue < i64 , Info >) , Topo (Vec < (i64 , Info) >) , }
    };
}

Queue!()