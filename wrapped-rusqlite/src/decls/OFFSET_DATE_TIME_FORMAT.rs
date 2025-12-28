macro_rules! OFFSET_DATE_TIME_FORMAT {
    () => {
        const OFFSET_DATE_TIME_FORMAT : & [FormatItem < '_ >] = format_description ! (version = 2 , "[year]-[month]-[day][first [ ][T]][hour]:[minute][optional [:[second][optional [.[subsecond]]]]][offset_hour sign:mandatory]:[offset_minute]") ;
    };
}

OFFSET_DATE_TIME_FORMAT!()