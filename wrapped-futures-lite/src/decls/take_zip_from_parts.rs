macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! take_zip_from_parts {
    () => {
        deps!();
        # [doc = " Extracts the contents of two options and zips them, handling `(Some(_), None)` cases"] fn take_zip_from_parts < T1 , T2 > (o1 : & mut Option < T1 > , o2 : & mut Option < T2 >) -> Poll < (T1 , T2) > { match (o1 . take () , o2 . take ()) { (Some (t1) , Some (t2)) => Poll :: Ready ((t1 , t2)) , (o1x , o2x) => { * o1 = o1x ; * o2 = o2x ; Poll :: Pending } } }
    };
}

take_zip_from_parts!();