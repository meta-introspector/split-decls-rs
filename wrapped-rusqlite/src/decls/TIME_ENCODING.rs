macro_rules! TIME_ENCODING {
    () => {
        const TIME_ENCODING : & [FormatItem < '_ >] = format_description ! (version = 2 , "[hour]:[minute]:[second].[subsecond]") ;
    };
}

TIME_ENCODING!()