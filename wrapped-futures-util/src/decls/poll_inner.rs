macro_rules! deps {
    () => {
        PollNext!();
        Pending!();
        Ready!();
    };
}

macro_rules! poll_inner {
    () => {
        deps!();
        # [inline] fn poll_inner < St1 , St2 , Clos , State > (select : & mut SelectWithStrategyProj < '_ , St1 , St2 , Clos , State > , side : PollNext , cx : & mut Context < '_ > ,) -> Poll < Option < St1 :: Item > > where St1 : Stream , St2 : Stream < Item = St1 :: Item > , { let first_done = match poll_side (select , side , cx) { Poll :: Ready (Some (item)) => return Poll :: Ready (Some (item)) , Poll :: Ready (None) => { select . internal_state . finish (side) ; true } Poll :: Pending => false , } ; let other = side . other () ; match poll_side (select , other , cx) { Poll :: Ready (None) => { select . internal_state . finish (other) ; if first_done { Poll :: Ready (None) } else { Poll :: Pending } } a => a , } }
    };
}

poll_inner!();