macro_rules! UTC_DATE_TIME_FORMAT {
    () => {
        const UTC_DATE_TIME_FORMAT : & [FormatItem < '_ >] = format_description ! (version = 2 , "[year]-[month]-[day][first [ ][T]][hour]:[minute][optional [:[second][optional [.[subsecond]]]]][optional [Z]]") ;
    };
}

UTC_DATE_TIME_FORMAT!()