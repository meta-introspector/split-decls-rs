macro_rules! PRIMITIVE_DATE_TIME_FORMAT {
    () => {
        const PRIMITIVE_DATE_TIME_FORMAT : & [FormatItem < '_ >] = format_description ! (version = 2 , "[year]-[month]-[day][first [ ][T]][hour]:[minute][optional [:[second][optional [.[subsecond]]]]]") ;
    };
}

PRIMITIVE_DATE_TIME_FORMAT!();