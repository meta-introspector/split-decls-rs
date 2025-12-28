macro_rules! PRIMITIVE_DATE_TIME_ENCODING {
    () => {
        const PRIMITIVE_DATE_TIME_ENCODING : & [FormatItem < '_ >] = format_description ! (version = 2 , "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]") ;
    };
}

PRIMITIVE_DATE_TIME_ENCODING!();