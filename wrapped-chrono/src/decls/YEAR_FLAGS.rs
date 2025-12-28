macro_rules! deps {
    () => {
        Weekday!();
        YearFlags!();
    };
}

macro_rules! YEAR_FLAGS {
    () => {
        deps!();
        const YEAR_FLAGS : [(i32 , YearFlags , Weekday) ; 14] = [(2006 , A , Weekday :: Sun) , (2005 , B , Weekday :: Sat) , (2010 , C , Weekday :: Fri) , (2009 , D , Weekday :: Thu) , (2003 , E , Weekday :: Wed) , (2002 , F , Weekday :: Tue) , (2001 , G , Weekday :: Mon) , (2012 , AG , Weekday :: Sun) , (2000 , BA , Weekday :: Sat) , (2016 , CB , Weekday :: Fri) , (2004 , DC , Weekday :: Thu) , (2020 , ED , Weekday :: Wed) , (2008 , FE , Weekday :: Tue) , (2024 , GF , Weekday :: Mon) ,] ;
    };
}

YEAR_FLAGS!()