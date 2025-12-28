macro_rules! deps {
    () => {
        FluentResource!();
        ResolverError!();
        EntryKind!();
    };
}

macro_rules! FluentError {
    () => {
        deps!();
        # [doc = " Core error type for Fluent runtime system."] # [doc = ""] # [doc = " It contains three main types of errors that may come up"] # [doc = " during runtime use of the fluent-bundle crate."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum FluentError { # [doc = " An error which occurs when"] # [doc = " [`FluentBundle::add_resource`](crate::bundle::FluentBundle::add_resource)"] # [doc = " adds entries that are already registered in a given [`FluentBundle`](crate::FluentBundle)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_bundle::{FluentBundle, FluentResource};"] # [doc = " use unic_langid::langid;"] # [doc = ""] # [doc = " let ftl_string = String::from(\"intro = Welcome, { $name }.\");"] # [doc = " let res1 = FluentResource::try_new(ftl_string)"] # [doc = "     .expect(\"Could not parse an FTL string.\");"] # [doc = ""] # [doc = " let ftl_string = String::from(\"intro = Hi, { $name }.\");"] # [doc = " let res2 = FluentResource::try_new(ftl_string)"] # [doc = "     .expect(\"Could not parse an FTL string.\");"] # [doc = ""] # [doc = " let langid_en = langid!(\"en-US\");"] # [doc = " let mut bundle = FluentBundle::new(vec![langid_en]);"] # [doc = ""] # [doc = " bundle.add_resource(&res1)"] # [doc = "     .expect(\"Failed to add FTL resources to the bundle.\");"] # [doc = ""] # [doc = " assert!(bundle.add_resource(&res2).is_err());"] # [doc = " ```"] Overriding { kind : EntryKind , id : String , } , ParserError (ParserError) , ResolverError (ResolverError) , }
    };
}

FluentError!();