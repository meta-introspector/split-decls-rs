macro_rules! deps {
    () => {
        SourceMap!();
        Annotation!();
        Message!();
        SplicedLines!();
        DisplaySuggestion!();
        Patch!();
        Padding!();
        AnnotatedLineInfo!();
        Origin!();
        Snippet!();
    };
}

macro_rules! PreProcessedElement {
    () => {
        deps!();
        enum PreProcessedElement < 'a > { Message (& 'a Message < 'a >) , Cause ((& 'a Snippet < 'a , Annotation < 'a > > , SourceMap < 'a > , Vec < AnnotatedLineInfo < 'a > > ,) ,) , Suggestion ((& 'a Snippet < 'a , Patch < 'a > > , SourceMap < 'a > , SplicedLines < 'a > , DisplaySuggestion ,) ,) , Origin (& 'a Origin < 'a >) , Padding (Padding) , }
    };
}

PreProcessedElement!();