macro_rules! QuoteOption {
    () => {
        struct QuoteOption < T > (Option < T >) ;
    };
}

QuoteOption!()