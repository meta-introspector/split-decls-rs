macro_rules! deps {
    () => {
        Graph!();
    };
}

macro_rules! get_graph_order_as_bits {
    () => {
        deps!();
        fn get_graph_order_as_bits (order : usize) -> Vec < usize > { let to_convert_to_bits = if order < N { vec ! [(order , 6)] } else if order <= 258047 { vec ! [(N , 6) , (order , 18)] } else { panic ! ("Graph order not supported.") } ; to_convert_to_bits . iter () . flat_map (| & (n , n_of_bits) | get_number_as_bits (n , n_of_bits)) . collect () }
    };
}

get_graph_order_as_bits!()