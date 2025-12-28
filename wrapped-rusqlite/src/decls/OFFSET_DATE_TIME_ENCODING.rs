macro_rules! OFFSET_DATE_TIME_ENCODING {
    () => {
        const OFFSET_DATE_TIME_ENCODING : & [FormatItem < '_ >] = format_description ! (version = 2 , "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond][offset_hour sign:mandatory]:[offset_minute]") ;
    };
}

OFFSET_DATE_TIME_ENCODING!()