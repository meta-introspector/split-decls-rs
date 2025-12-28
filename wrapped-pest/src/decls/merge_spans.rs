macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! merge_spans {
    () => {
        deps!();
        # [doc = " Merges two spans into one."] # [doc = ""] # [doc = " This function merges two spans that are contiguous or overlapping into a single span"] # [doc = " that covers the entire range of the two input spans. This is useful when you want to"] # [doc = " aggregate information from multiple spans into a single entity."] # [doc = ""] # [doc = " The function checks if the input spans are overlapping or contiguous by comparing their"] # [doc = " start and end positions. If they are, a new span is created with the minimum start position"] # [doc = " and the maximum end position of the two input spans."] # [doc = ""] # [doc = " If the input spans are neither overlapping nor contiguous, the function returns None,"] # [doc = " indicating that a merge operation was not possible."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use pest;"] # [doc = " # use pest::Span;"] # [doc = " # use pest::merge_spans;"] # [doc = ""] # [doc = " // Example 1: Contiguous spans"] # [doc = " let input = \"abc\\ndef\\nghi\";"] # [doc = " let span1 = Span::new(input, 1, 7).unwrap();"] # [doc = " let span2 = Span::new(input, 7, 11).unwrap();"] # [doc = " let merged = merge_spans(&span1, &span2).unwrap();"] # [doc = " assert_eq!(merged, Span::new(input, 1, 11).unwrap());"] # [doc = ""] # [doc = " // Example 2: Overlapping spans"] # [doc = " let input = \"abc\\ndef\\nghi\";"] # [doc = " let span1 = Span::new(input, 1, 7).unwrap();"] # [doc = " let span2 = Span::new(input, 5, 11).unwrap();"] # [doc = " let merged = merge_spans(&span1, &span2).unwrap();"] # [doc = " assert_eq!(merged, Span::new(input, 1, 11).unwrap());"] # [doc = ""] # [doc = " // Example 3: Non-contiguous spans"] # [doc = " let input = \"abc\\ndef\\nghi\";"] # [doc = " let span1 = Span::new(input, 1, 7).unwrap();"] # [doc = " let span2 = Span::new(input, 8, 11).unwrap();"] # [doc = " let merged = merge_spans(&span1, &span2);"] # [doc = " assert!(merged.is_none());"] # [doc = " ```"] pub fn merge_spans < 'i > (a : & Span < 'i > , b : & Span < 'i >) -> Option < Span < 'i > > { if a . end () >= b . start () && a . start () <= b . end () { Span :: new (a . get_input () , core :: cmp :: min (a . start () , b . start ()) , core :: cmp :: max (a . end () , b . end ()) ,) } else { None } }
    };
}

merge_spans!()