macro_rules! PRIMITIVE_DATE_TIME_FORMAT {
    () => {
        const PRIMITIVE_DATE_TIME_FORMAT : & [FormatItem < '_ >] = format_description ! ("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]") ;
    };
}

PRIMITIVE_DATE_TIME_FORMAT!();