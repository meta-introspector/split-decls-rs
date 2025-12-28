macro_rules! deps {
    () => {
        Key!();
        State!();
        Duration!();
    };
}

macro_rules! Throughput {
    () => {
        deps!();
        # [doc = " A utility to compute throughput of a set of progress values usually available to a renderer."] # [derive (Default)] pub struct Throughput { sorted_by_key : Vec < (progress :: Key , State) > , updated_at : Option < SystemTime > , elapsed : Option < Duration > , }
    };
}

Throughput!();