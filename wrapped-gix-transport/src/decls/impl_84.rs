macro_rules! deps {
    () => {
        GetResponse!();
        PostResponse!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < A , B , C > From < PostResponse < A , B , C > > for GetResponse < A , B > { fn from (v : PostResponse < A , B , C >) -> Self { GetResponse { headers : v . headers , body : v . body , } } }
    };
}

impl_84!();