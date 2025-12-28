macro_rules! clear_and_set_capacity {
    () => {
        fn clear_and_set_capacity (buf : & mut Vec < u8 > , cap : usize) -> Result < () , std :: collections :: TryReserveError > { buf . clear () ; if buf . capacity () < cap { buf . try_reserve (cap) ? ; debug_assert ! (buf . capacity () >= cap , "{} >= {}" , buf . capacity () , cap) ; } Ok (()) }
    };
}

clear_and_set_capacity!()