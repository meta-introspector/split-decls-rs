macro_rules! TIME_FORMAT {
    () => {
        const TIME_FORMAT : & [FormatItem < '_ >] = format_description ! (version = 2 , "[hour]:[minute][optional [:[second][optional [.[subsecond]]]]]") ;
    };
}

TIME_FORMAT!()