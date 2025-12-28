macro_rules! deps {
    () => {
        PrintFrame!();
        RingBuffer!();
        BufEntry!();
    };
}

macro_rules! Printer {
    () => {
        deps!();
        pub struct Printer { out : String , space : isize , buf : RingBuffer < BufEntry > , left_total : isize , right_total : isize , scan_stack : VecDeque < usize > , print_stack : Vec < PrintFrame > , indent : usize , pending_indentation : usize , }
    };
}

Printer!();