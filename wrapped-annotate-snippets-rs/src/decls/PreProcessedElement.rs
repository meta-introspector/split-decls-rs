macro_rules! deps {
    () => {
        Snippet!();
        Message!();
        Origin!();
        AnnotatedLineInfo!();
        Padding!();
        Annotation!();
        SourceMap!();
        DisplaySuggestion!();
        SplicedLines!();
        Patch!();
    };
}

macro_rules! PreProcessedElement {
    () => {
        deps!();
        enum PreProcessedElement < 'a > { Message (& 'a Message < 'a >) , Cause ((& 'a Snippet < 'a , Annotation < 'a > > , SourceMap < 'a > , Vec < AnnotatedLineInfo < 'a > > ,) ,) , Suggestion ((& 'a Snippet < 'a , Patch < 'a > > , SourceMap < 'a > , SplicedLines < 'a > , DisplaySuggestion ,) ,) , Origin (& 'a Origin < 'a >) , Padding (Padding) , }
    };
}

PreProcessedElement!()