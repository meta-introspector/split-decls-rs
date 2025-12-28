macro_rules! Error {
    () => {
        # [derive (thiserror :: Error , Debug , Clone)] # [allow (missing_docs)] pub enum Error { # [error ("Could not convert a duration into a date")] RelativeTimeConversion , # [error ("Date string can not be parsed")] InvalidDateString { input : String } , # [error ("The heat-death of the universe happens before this date")] InvalidDate (# [from] std :: num :: TryFromIntError) , # [error ("Current time is missing but required to handle relative dates.")] MissingCurrentTime , }
    };
}

Error!();