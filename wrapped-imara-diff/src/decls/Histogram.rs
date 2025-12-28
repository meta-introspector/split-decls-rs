macro_rules! Histogram {
    () => {
        struct Histogram { token_occurrences : Vec < ListHandle > , pool : ListPool , }
    };
}

Histogram!()