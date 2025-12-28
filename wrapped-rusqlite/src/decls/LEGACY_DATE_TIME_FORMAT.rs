macro_rules! LEGACY_DATE_TIME_FORMAT {
    () => {
        const LEGACY_DATE_TIME_FORMAT : & [FormatItem < '_ >] = format_description ! (version = 2 , "[year]-[month]-[day] [hour]:[minute]:[second]:[subsecond] [offset_hour sign:mandatory]:[offset_minute]") ;
    };
}

LEGACY_DATE_TIME_FORMAT!();