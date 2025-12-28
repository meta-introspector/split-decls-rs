macro_rules! ReportLink {
    () => {
        # [derive (Serialize , Debug)] struct ReportLink < 'a > { name : & 'a str , path : Option < String > , }
    };
}

ReportLink!()